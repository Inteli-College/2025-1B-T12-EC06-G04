from ultralytics import YOLO

def rodar_modelo(path, modelPath):
    model = YOLO(modelPath)
    results = model.predict(source=path, save=True, save_txt=True, save_conf=True, show=True)
    