from ultralytics import YOLO
import json
from collections import defaultdict

def rodar_modelo(dir_path, model_path):
    label_to_name = {0: "retracao", 1: "termica"}

    model = YOLO(model_path)
    results = model.predict(source=dir_path)

    # dicionário intermediário: chave = caminho, valor = lista de fissuras
    per_image = defaultdict(lambda: {"path": None, "fissura": []})

    for res in results:
        img_path = res.path
        if per_image[img_path]["path"] is None:
            per_image[img_path]["path"] = img_path

        # iterar pareando classe e confiança corretamente
        for cls, conf in zip(res.boxes.cls.tolist(), res.boxes.conf.tolist()):
            per_image[img_path]["fissura"].append(
                {"name": label_to_name[int(cls)], "confidence": float(conf)}
            )

    return list(per_image.values())

with open("resultados.json", "w") as f:
    f.write(
        json.dumps(rodar_modelo("RunDataset", "best.pt"), indent=4, ensure_ascii=False)
    )