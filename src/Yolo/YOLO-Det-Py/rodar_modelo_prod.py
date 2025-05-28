from ultralytics import YOLO

def rodar_modelo(path, modelPath):
    labelToName = {
        0: "retracao",
        1: "termica"}
    model = YOLO(modelPath)
    results = model.predict(source=path)
    final = []
    for result in results:
        path = result.path
        labels = result.boxes.cls.tolist()
        confidence  = result.boxes.conf.tolist()
        for label in labels:
            name = labelToName[label]
            preResultado = {"path": path, "fissura":[]}
            preResultado["fissura"].append({"name": name, "confidence": confidence[labels.index(label)]})
            final.append(preResultado)
    return final
print(rodar_modelo("RunDataset", "best.pt"))