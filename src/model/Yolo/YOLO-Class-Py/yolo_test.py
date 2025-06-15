from ultralytics import YOLO

model = YOLO("bs.pt")
model.val()