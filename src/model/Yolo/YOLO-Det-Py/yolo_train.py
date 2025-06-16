from ultralytics import YOLO

model = YOLO("yolov8m.pt")

model.train(
    data='Dataset/dataset.yaml',
    epochs=30,
    imgsz=224,
    batch=32,
    name="yolo_rachaduras"
)