from ultralytics import YOLO

model = YOLO("best2.pt")

model.train(
    data='Dataset/dataset.yaml',
    epochs=30,
    imgsz=224,
    batch=16,
    warmup_epochs=3,
    mosaic=1.0,
    mixup=0.2,
    cutmix=0.2,
    hsv_h=0.015, hsv_s=0.7, hsv_v=0.4,
    perspective=0.0008, shear=2.0,
    name="yolo_rachaduras_recall"
)
