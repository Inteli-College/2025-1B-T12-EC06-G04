from ultralytics import YOLO

model = YOLO("yolov8n.pt")

model.train(
    data='Dataset/dataset.yaml',
    epochs=200,
    imgsz=224,
    batch=16,
    name="reclasse_v8n",
)
