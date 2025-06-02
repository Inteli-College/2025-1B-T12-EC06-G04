import argparse
from ultralytics import YOLO
import json
from collections import defaultdict
import os # Ensure os is imported
import sys # Import sys for stderr

def rodar_modelo(dir_path, model_path):
    print(f"Carregando modelo de: {model_path}", file=sys.stderr)
    label_to_name = {0: "retracao", 1: "termica"}

    model = YOLO(model_path)

    # Construct a recursive glob pattern.
    glob_pattern = os.path.join(dir_path, '**', '*')
    
    print(f"Rodando predição com o padrão glob para busca recursiva: {glob_pattern}", file=sys.stderr)
    print(f"Diretório base sendo pesquisado (caminho absoluto): {os.path.abspath(dir_path)}", file=sys.stderr)

    # Attempt to suppress Ultralytics' own stdout logging
    results = model.predict(source=glob_pattern, verbose=False)

    # dicionário intermediário: chave = caminho, valor = lista de fissuras
    per_image = defaultdict(lambda: {"path": None, "fissura": []})
    
    processed_something = False
    if results:
        for res in results:
            if hasattr(res, 'boxes') and res.boxes is not None and res.boxes.cls is not None: # Check if results have boxes attribute
                processed_something = True
                img_path = res.path
                if per_image[img_path]["path"] is None:
                    per_image[img_path]["path"] = img_path

                # iterar pareando classe e confiança corretamente
                for cls, conf in zip(res.boxes.cls.tolist(), res.boxes.conf.tolist()):
                    per_image[img_path]["fissura"].append(
                        {"name": label_to_name[int(cls)], "confidence": float(conf)}
                    )
            else:
                # Handle cases where a result object might not have detections (e.g. non-image file processed by glob)
                if hasattr(res, 'path'):
                    print(f"Aviso: Nenhum objeto 'boxes' encontrado para o arquivo processado: {res.path}. Pode não ser uma imagem suportada ou não houve detecções.", file=sys.stderr)
                else:
                    print(f"Aviso: Item processado sem atributo 'path' ou 'boxes'. Conteúdo: {res}", file=sys.stderr)


    if not processed_something:
        print(f"Nenhuma imagem foi processada com sucesso ou nenhuma detecção foi encontrada para o padrão: {glob_pattern}", file=sys.stderr)
        print(f"Verifique se o diretório '{os.path.abspath(dir_path)}' contém imagens nos formatos suportados (recursivamente).", file=sys.stderr)
        return [] # Return empty list if nothing was processed

    return list(per_image.values())

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Rodar modelo YOLO em um projeto específico.")
    parser.add_argument("project_name", type=str, help="Nome do projeto (a pasta de imagens estará em Projects/project_name/images)")
    parser.add_argument("model_path", type=str, help="Caminho para o arquivo do modelo (ex: best.pt)")
    args = parser.parse_args()

    # Assuming the script CWD when run by Rust is 'src/app-rust/'
    # Path to the WORKSPACE_ROOT/Projects/{project_name}
    project_base_dir = os.path.join("..", "Projects", args.project_name)
    # Path to WORKSPACE_ROOT/Projects/{project_name}/images
    image_dir_from_arg = os.path.join(project_base_dir, "images")
    # Path to WORKSPACE_ROOT/Projects/{project_name}/detection_results.json
    output_json_path = os.path.join(project_base_dir, "detection_results.json")
    
    print(f"Script CWD (for context): {os.getcwd()}", file=sys.stderr)
    print(f"Argumento project_name recebido: {args.project_name}", file=sys.stderr)
    print(f"Caminho relativo do diretório base do projeto (para output): {project_base_dir}", file=sys.stderr)
    print(f"Caminho relativo do diretório de imagens (para input): {image_dir_from_arg}", file=sys.stderr)
    print(f"Caminho do arquivo JSON de saída (relativo): {output_json_path}", file=sys.stderr)
        
    # Get absolute paths for clarity and for os.path.isdir check
    abs_image_dir = os.path.abspath(image_dir_from_arg)
    abs_output_json_path = os.path.abspath(output_json_path)
    abs_project_base_dir = os.path.abspath(project_base_dir)

    print(f"Caminho absoluto do diretório de imagens que será processado: {abs_image_dir}", file=sys.stderr)
    print(f"Caminho absoluto do JSON de saída: {abs_output_json_path}", file=sys.stderr)

    if not os.path.isdir(abs_image_dir):
        print(f"ERRO: O diretório de imagens especificado não existe ou não é um diretório: {abs_image_dir}", file=sys.stderr)
        print("[]") 
        exit(1)

    results_list = rodar_modelo(abs_image_dir, args.model_path) # Pass absolute path here
    
    # Save the results to a JSON file
    try:
        os.makedirs(abs_project_base_dir, exist_ok=True)
        with open(abs_output_json_path, 'w', encoding='utf-8') as f:
            json.dump(results_list, f, indent=4, ensure_ascii=False)
        print(f"Resultados da detecção salvos em: {abs_output_json_path}", file=sys.stderr)
    except IOError as e:
        print(f"ERRO: Não foi possível salvar o arquivo JSON em {abs_output_json_path}: {e}", file=sys.stderr)

    if results_list:
        print(f"Predição concluída. {len(results_list)} imagens com informações de fissura.", file=sys.stderr)
    else:
        print("Nenhuma informação de fissura foi retornada pela predição.", file=sys.stderr)
        
    # This print still goes to stdout for the Rust application to capture
    print(json.dumps(results_list, indent=4, ensure_ascii=False))