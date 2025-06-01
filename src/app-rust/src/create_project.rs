use dioxus::prelude::*;
use std::path::PathBuf;
use dioxus_router::prelude::Link;
use crate::Route;

#[component]
pub fn NewProject() -> Element {
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut year = use_signal(|| "".to_string());
    let mut leader = use_signal(|| String::new());
    let mut structure_type = use_signal(|| String::new());
    let mut observations = use_signal(|| String::new());
    let mut status = use_signal(|| String::new());
    let mut is_creating = use_signal(|| false);

    let create_project = move |_| {
        if name().trim().is_empty() || year().trim().is_empty() {
            status.set("Por favor, preencha nome e ano.".to_string());
            return;
        }

        is_creating.set(true);
        let project_name = name().trim().to_string();
        let project_year = year().trim().to_string();

        spawn(async move {
            // Caminho base onde os projetos serão salvos
            let base_path = PathBuf::from("Projetos");
            let new_folder = base_path.join(format!("{} - {}", project_year, project_name));

            if let Err(e) = std::fs::create_dir_all(&new_folder) {
                status.set(format!("Erro ao criar pasta: {}", e));
            } else {
                status.set(format!("Projeto criado em: {}", new_folder.display()));
                // Aqui você pode redirecionar ou limpar o formulário
            }

            is_creating.set(false);
        });
    };

    rsx! {
        div { class: "min-h-screen bg-gray-100 text-gray-900 font-sans",
            div { class: "container mx-auto px-4 py-12 max-w-2xl",
                h1 { class: "text-3xl font-bold text-center mb-8", "Criar Novo Projeto" }

                div { class: "bg-white rounded-lg shadow-md p-6 space-y-6",

                    div {
                        label { class: "block text-gray-700 mb-1", "Nome do Projeto" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{name()}",
                            oninput: move |e| name.set(e.value())
                        }
                    }

                    div {
                        label { class: "block text-gray-700 mb-1", "Descrição" }
                        textarea {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            value: "{description()}",
                            rows: "4",
                            oninput: move |e| description.set(e.value())
                        }
                    }

                    div {
                        label { class: "block text-gray-700 mb-1", "Líder responsável pelo projeto" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{leader()}",
                            oninput: move |e| leader.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-gray-700 mb-1", "Tipo de estrutura do edifício" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{structure_type()}",
                            oninput: move |e| structure_type.set(e.value())
                        }
                    }

                    div {
                        label { class: "block text-gray-700 mb-1", "Ano" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "number",
                            value: "{year()}",
                            min: "1800",
                            max: "2100",
                            oninput: move |e| year.set(e.value())
                        }
                    }

                    div {
                        label { class: "block text-gray-700 mb-1", "Observações gerais" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{observations()}",
                            oninput: move |e| observations.set(e.value())
                        }
                    }

                    button {
                        class: "w-full px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: is_creating(),
                        onclick: create_project,
                        if is_creating() { "Criando projeto..." } else { "Criar Projeto" }
                    }

                    if !status().is_empty() {
                        p { class: "text-center text-gray-700", "{status()}" }
                        Link {
                            to: Route::HomePage {},
                            button {
                                class: "fixed bottom-6 right-6 bg-purple-100 hover:bg-purple-200 text-purple-600 shadow-lg p-4 rounded-full",
                                title: "Voltar para a página inicial",
                                i { class: "material-icons", "arrow_back" }
                            }
                        }
                    }
                }
            }
        }
    }
}
