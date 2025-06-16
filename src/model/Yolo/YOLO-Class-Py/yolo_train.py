from ultralytics import YOLO

model = YOLO("yolov8n-cls.pt")

model.train(
    data='Imagens',
    epochs=30,
    imgsz=224,
    batch=32,
    name="yolo_rachaduras"
)