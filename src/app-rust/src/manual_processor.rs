use dioxus::prelude::*;
use std::collections::HashMap;
use rfd::AsyncFileDialog;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::fs;

#[derive(Clone, PartialEq)]
pub struct Building {
    pub name: String,
    pub facades: HashMap<String, Vec<ImageData>>, // Key is facade name
}

#[derive(Clone, PartialEq)]
pub struct ImageData {
    pub path: String,
    pub name: String,
    pub preview_url: Option<String>,
}

#[component]
pub fn ManualProcessor() -> Element {
    let mut num_buildings = use_signal(|| 1);
    let mut buildings = use_signal(Vec::<Building>::new); // Use ::new for default
    let mut selected_building = use_signal(|| 0);
    let mut building_names = use_signal(|| vec!["Prédio 1".to_string()]);
    let mut facade_counts = use_signal(|| vec![1]);
    // Ensure HashMap is properly initialized for each building's facade names
    let mut facade_names = use_signal(|| vec![HashMap::new()]); 
    let mut is_processing = use_signal(|| false);
    let mut status = use_signal(String::new); // Use ::new for default

    use_effect(move || {
        let current_num_buildings_val = *num_buildings.read();
        let mut desired_num_buildings = current_num_buildings_val;

        // Sanitize num_buildings
        if desired_num_buildings > 20 {
            desired_num_buildings = 20;
        }
        // Ensure num_buildings is at least 1
        if desired_num_buildings < 1 {
            desired_num_buildings = 1;
        }

        // If num_buildings was adjusted, set it and let the effect re-run with the new value.
        if current_num_buildings_val != desired_num_buildings {
            num_buildings.set(desired_num_buildings);
            return;
        }

        // At this point, desired_num_buildings holds the validated and stable count.
        let count = desired_num_buildings;

        // Clone all the data we need before any writes
        let current_buildings = buildings.read().clone();
        let current_building_names = building_names.read().clone();
        let current_facade_counts = facade_counts.read().clone();
        let current_facade_names = facade_names.read().clone();

        // Check if we need to update
        if current_buildings.len() == count {
            // Only adjust selected_building if needed
            if *selected_building.read() >= count && count > 0 {
                selected_building.set(count - 1);
            } else if count == 0 {
                selected_building.set(0);
            }
            return;
        }

        // Create new vectors with the desired capacity
        let mut new_buildings_vec = Vec::with_capacity(count);
        let mut new_building_names_vec = Vec::with_capacity(count);
        let mut new_facade_counts_vec = Vec::with_capacity(count);
        let mut new_facade_names_vec = Vec::with_capacity(count);

        // Populate the new vectors using the cloned data
        for i in 0..count {
            let name = current_building_names.get(i)
                .cloned()
                .unwrap_or_else(|| format!("Prédio {}", i + 1));
            
            let facades_data = current_buildings.get(i)
                .map(|b| b.facades.clone())
                .unwrap_or_else(HashMap::new);

            new_buildings_vec.push(Building {
                name: name.clone(),
                facades: facades_data,
            });
            new_building_names_vec.push(name);
            new_facade_counts_vec.push(current_facade_counts.get(i).cloned().unwrap_or(1));
            new_facade_names_vec.push(current_facade_names.get(i).cloned().unwrap_or_else(HashMap::new));
        }

        // Now we can safely write to all signals since we're not holding any read locks
        buildings.set(new_buildings_vec);
        building_names.set(new_building_names_vec);
        facade_counts.set(new_facade_counts_vec);
        facade_names.set(new_facade_names_vec);

        // Adjust selected_building if needed
        if *selected_building.read() >= count && count > 0 {
            selected_building.set(count - 1);
        } else if count == 0 {
            selected_building.set(0);
        }
    });

    let organize_folders = move |_| {
        // Clone necessary data for the async block
        let current_buildings_for_async = buildings.read().clone(); 
        // Make sure is_processing and status signals are properly captured if needed by value for set
        let mut is_processing_writer = is_processing;
        let mut status_writer = status;

        spawn(async move {
            is_processing_writer.set(true);
            status_writer.set("Organizando pastas...".to_string());
            
            // Iterate over the cloned data
            for building_detail in current_buildings_for_async.iter() {
                let building_folder_name = &building_detail.name;
                // It's good practice to ensure folder names are sanitized
                // For simplicity, using as is.
                let building_path = PathBuf::from(building_folder_name);
                
                if let Err(e) = fs::create_dir_all(&building_path) {
                    status_writer.set(format!("Erro ao criar pasta do prédio {}: {}", building_folder_name, e));
                    // Decide if you want to stop all processing or continue with other buildings
                    // For now, setting is_processing to false and returning.
                    is_processing_writer.set(false);
                    return;
                }
                
                for (facade_folder_name, images) in &building_detail.facades {
                    let facade_path = building_path.join(facade_folder_name);
                    
                    if let Err(e) = fs::create_dir_all(&facade_path) {
                        status_writer.set(format!("Erro ao criar pasta da fachada {} (prédio {}): {}", facade_folder_name, building_folder_name, e));
                        continue; // Continue with the next facade or image
                    }
                    
                    for image_data in images {
                        let source_path = PathBuf::from(&image_data.path);
                        // Ensure image_data.name is a valid file name
                        let target_path = facade_path.join(&image_data.name);
                        
                        // Check if source_path exists before trying to copy
                        if !source_path.exists() {
                            status_writer.set(format!("Erro: Imagem de origem não encontrada {} para {}", image_data.path, facade_folder_name));
                            continue; // Skip this image
                        }

                        if let Err(e) = fs::copy(&source_path, &target_path) {
                            status_writer.set(format!("Erro ao copiar imagem {} para {}: {}", image_data.name, facade_folder_name, e));
                            // Decide if this error should stop everything or just log and continue
                        }
                    }
                }
            }
            
            status_writer.set("Organização concluída com sucesso!".to_string());
            is_processing_writer.set(false);
            
            // Close the window after successful organization
            dioxus::desktop::window().close();
        });
    };

    rsx! {
        div { class: "min-h-screen bg-gray-100 text-gray-900 font-sans",
            document::Stylesheet { href: asset!("/assets/tailwind.css") }
            document::Link {
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
                rel: "stylesheet"
            }

            div { class: "container mx-auto px-4 py-8 max-w-4xl",
                h1 { class: "text-3xl font-bold text-center mb-8 text-gray-800", 
                    "Processamento Manual de Imagens" 
                }
                
                div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                    div { class: "mb-6",
                        label { class: "block text-gray-700 mb-2", 
                            "Número de Prédios (1-20):" 
                        }
                        select {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white",
                            value: "{num_buildings()}",
                            onchange: move |e| {
                                if let Ok(val) = e.value().parse::<usize>() {
                                    if val > 0 && val <= 20 {
                                        num_buildings.set(val);
                                    }
                                }
                            },
                            for i in 1..=20 {
                                option { key: "{i}", value: "{i}", "{i}" }
                            }
                        }
                    }

                    div { class: "space-y-6",
                        { // Start of Rust block for iterating buildings
                            // Corrected: Signal reads are moved inside the .map() closure below
                            (0..num_buildings()).map(|i| { // Iterate buildings
                                // Read signals inside the map closure for each iteration 'i'
                                let building_names_data = building_names.read();
                                let facade_counts_data = facade_counts.read();
                                let facade_names_data = facade_names.read();
                                let buildings_data = buildings.read();

                                if i < building_names_data.len() && 
                                   i < facade_counts_data.len() && 
                                   i < facade_names_data.len() &&
                                   i < buildings_data.len() {
                                    // Clone the specific data needed for this iteration if it's not Copy.
                                    // For `usize` (facade_count_val), it's Copy.
                                    // For `String` (building_name_val), clone is good.
                                    let building_name_val = building_names_data[i].clone();
                                    let facade_count_val = facade_counts_data[i]; 
                                    // facade_names_data and buildings_data (the guards) will be used by the inner loop.
                                    // Their lifetime is now tied to this outer .map() iteration.
                                    
                                    rsx! { // RSX for a single building
                                        document::Stylesheet { href: asset!("/assets/tailwind.css") }
                                        div { key: "building-{i}", class: "bg-white rounded-lg shadow-md p-6 mb-6",
                                            div { class: "mb-6", // Building Name
                                                label { class: "block text-gray-700 mb-2", "Nome do Prédio:" }
                                                input {
                                                    class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                                    value: "{building_name_val}",
                                                    oninput: move |e| {
                                                        let new_building_name = e.value();
                                                        // When writing, get a fresh write guard
                                                        building_names.write()[i] = new_building_name.clone();
                                                        buildings.write()[i].name = new_building_name;
                                                    }
                                                }
                                            }
            
                                            div { class: "mb-6", // Facade Count
                                                label { class: "block text-gray-700 mb-2", "Número de Fachadas (1-8):" }
                                                select {
                                                    class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white",
                                                    value: "{facade_count_val}",
                                                    onchange: move |e| {
                                                        if let Ok(val) = e.value().parse::<usize>() {
                                                            if val > 0 && val <= 8 {
                                                                // When writing, get a fresh write guard
                                                                facade_counts.write()[i] = val;
                                                                facade_names.write()[i].retain(|&k, _| k < val);
                                                            }
                                                        }
                                                    },
                                                    for j_opt in 1..=8 {
                                                        option { key: "facade-opt-{i}-{j_opt}", value: "{j_opt}", "{j_opt}" }
                                                    }
                                                }
                                            }
            
                                            div { class: "space-y-4", // Facades list for this building
                                                { // Braces for the iterator expression
                                                    (0..facade_count_val).map(|j| { // j is the UI index for the facade
                                                        // `facade_names_data` (guard) is from the outer .map() iteration's scope
                                                        let current_facade_name_for_ui = facade_names_data.get(i)
                                                            .and_then(|map| map.get(&j))
                                                            .cloned()
                                                            .unwrap_or_else(|| format!("Fachada {}", j + 1));

                                                        rsx! { // RSX for a single facade
                                                            document::Stylesheet { href: asset!("/assets/tailwind.css") }
                                                            div { key: "facade-{i}-{j}", class: "bg-gray-50 rounded-lg p-4",
                                                                div { class: "flex items-center justify-between mb-4", // Facade Name and Add Images
                                                                    div { class: "flex-1 mr-4",
                                                                        label { class: "block text-gray-700 mb-2", "Nome da Fachada:" }
                                                                        input {
                                                                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                                                            placeholder: "Nome da fachada",
                                                                            value: "{current_facade_name_for_ui}",
                                                                            oninput: move |e| {
                                                                                let new_facade_name_ui = e.value();
                                                                                // Get fresh write guard for facade_names
                                                                                let mut current_facade_names_map_vec = facade_names.write();
                                                                                let old_name_ui = current_facade_names_map_vec[i]
                                                                                    .get(&j)
                                                                                    .cloned()
                                                                                    .unwrap_or_else(|| format!("Fachada {}", j + 1));

                                                                                current_facade_names_map_vec[i].insert(j, new_facade_name_ui.clone());
                    
                                                                                // Get fresh write guard for buildings
                                                                                let mut current_buildings_vec = buildings.write();
                                                                                if i < current_buildings_vec.len() {
                                                                                    if old_name_ui != new_facade_name_ui {
                                                                                        if let Some(images) = current_buildings_vec[i].facades.remove(&old_name_ui) {
                                                                                            current_buildings_vec[i].facades.insert(new_facade_name_ui.clone(), images);
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    button { // Add Images button
                                                                        class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 flex items-center gap-2",
                                                                        onclick: move |_| {
                                                                            let building_idx = i; // Capture i from outer map
                                                                            let facade_ui_idx_for_add = j; // Capture j from inner map
                                                                            // Signals for reading/writing within spawn should be handled carefully, often by cloning or re-reading.
                                                                            // Here, facade_names is read, and buildings is written.
                                                                            spawn(async move {
                                                                                if let Some(files) = AsyncFileDialog::new()
                                                                                    .add_filter("Imagens", &["jpg", "jpeg", "png"])
                                                                                    .pick_files()
                                                                                    .await
                                                                                {
                                                                                    // Re-read facade_names inside async block if its value might change due to user input while file dialog is open.
                                                                                    // Or use the captured facade_name_key if that's intended.
                                                                                    let facade_name_key = facade_names.read()[building_idx]
                                                                                        .get(&facade_ui_idx_for_add)
                                                                                        .cloned()
                                                                                        .unwrap_or_else(|| format!("Fachada {}", facade_ui_idx_for_add + 1));
                    
                                                                                    let image_data_vec: Vec<ImageData> = files
                                                                                        .iter()
                                                                                        .map(|f| ImageData {
                                                                                            path: f.path().display().to_string(),
                                                                                            name: f.file_name(), 
                                                                                            preview_url: None,
                                                                                        })
                                                                                        .collect();
                                                                                    
                                                                                    // Get fresh write guard for buildings
                                                                                    buildings.write()[building_idx].facades
                                                                                        .entry(facade_name_key)
                                                                                        .or_default()
                                                                                        .extend(image_data_vec);
                                                                                }
                                                                            });
                                                                        },
                                                                        i { class: "material-icons", "add_photo_alternate" }
                                                                        "Adicionar Imagens"
                                                                    }
                                                                } // End Facade Name and Add Images flexbox
                    
                                                                // Image grid for this facade
                                                                // `buildings_data` (guard) is from the outer .map() iteration's scope
                                                                if let Some(images_for_facade) = buildings_data[i].facades.get(&current_facade_name_for_ui) {
                                                                    if !images_for_facade.is_empty() {
                                                                        div { class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4",
                                                                            for (img_idx, img_data) in images_for_facade.iter().enumerate() {
                                                                                div { key: "img-{i}-{j}-{img_idx}", class: "relative group",
                                                                                    div { class: "aspect-w-4 aspect-h-3 bg-gray-200 rounded-lg overflow-hidden",
                                                                                        img {
                                                                                            src: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=",
                                                                                            class: "w-full h-full object-cover",
                                                                                            alt: "{img_data.name}"
                                                                                        }
                                                                                    }
                                                                                    div { class: "absolute inset-0 bg-black bg-opacity-50 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center", // Delete button overlay
                                                                                        button {
                                                                                            class: "text-white hover:text-red-500",
                                                                                            onclick: move |_| {
                                                                                                let building_idx_del = i; // Capture i
                                                                                                let facade_ui_idx_del = j; // Capture j
                                                                                                let image_index_to_remove = img_idx;
                    
                                                                                                let facade_name_key_for_delete = facade_names.read()[building_idx_del]
                                                                                                    .get(&facade_ui_idx_del)
                                                                                                    .cloned()
                                                                                                    .unwrap_or_else(|| format!("Fachada {}", facade_ui_idx_del + 1));
                    
                                                                                                // Get fresh write guard for buildings
                                                                                                let mut all_buildings = buildings.write();
                                                                                                if building_idx_del < all_buildings.len() {
                                                                                                    if let Some(facade_images_vec) = all_buildings[building_idx_del].facades.get_mut(&facade_name_key_for_delete) {
                                                                                                        if image_index_to_remove < facade_images_vec.len() {
                                                                                                            facade_images_vec.remove(image_index_to_remove);
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            },
                                                                                            i { class: "material-icons", "delete" }
                                                                                        }
                                                                                    }
                                                                                    p { class: "mt-2 text-sm text-gray-600 truncate", "{img_data.name}" }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } // End Image Grid
                                                            } // End single facade RSX
                                                        }
                                                    }) // End .map() for facades
                                                } // End of curly braces wrapping the map
                                            } // End Facades list div
                                        } // End single building RSX
                                    }
                                } else { // Else for `if i < data_lengths`
                                    rsx! { // Fallback for out-of-bounds (should ideally not happen if data is synced)
                                        document::Stylesheet { href: asset!("/assets/tailwind.css") }
                                        div { key: "loading-{i}", class: "bg-gray-100 rounded-lg p-4 animate-pulse",
                                            "Carregando dados do prédio..."
                                        }
                                    }
                                }
                            }) // End .map() for buildings
                        } // End of Rust block for iterating buildings
                    }

                    div { class: "mt-6 space-y-4", // Status and Confirm button
                        if !status().is_empty() {
                            p { class: "text-center text-gray-700", "{status()}" }
                        }
                        
                        div { class: "flex justify-end gap-4",
                            button {
                                class: "px-6 py-3 bg-green-600 text-white rounded-md hover:bg-green-700 flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed",
                                disabled: is_processing(),
                                onclick: organize_folders,
                                i { class: "material-icons", "check" }
                                if is_processing() { "Organizando..." } else { "Confirmar Organização" }
                            }
                        }
                    }
                }
            }
        }
    }
}