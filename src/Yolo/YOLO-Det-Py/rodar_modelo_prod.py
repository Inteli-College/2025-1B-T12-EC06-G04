from ultralytics import YOLO

def rodar_modelo(path, modelPath):
    labelToName = {
        0: "retracao",
        1: "termica"}
    model = YOLO(modelPath)
    results = model.predict(source=path, show=True)
    total = {0: 0, 1: 0}
    for result in results:
        labels = result.boxes.cls.tolist()
        for label in labels:
            total[label] += 1
        print(labels)
    print("Total de objetos detectados por classe:")
    for label, count in total.items():
        print(f"Classe {labelToName[label]}: {count} detectado(s)")
    return total
print(rodar_modelo("RunDataset", "best.pt"))