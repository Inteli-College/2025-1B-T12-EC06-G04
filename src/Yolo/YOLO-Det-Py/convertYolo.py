import os
import json
import cv2

# Define your classes and map them to numeric IDs
CLASS_MAP = {
    "retracao": 0,
    "termica": 1
}

def convert_labelme_folder(json_folder, image_folder, output_folder):
    os.makedirs(output_folder, exist_ok=True)

    for filename in os.listdir(json_folder):
        if not filename.endswith('.json'):
            continue

        json_path = os.path.join(json_folder, filename)
        with open(json_path, 'r') as f:
            data = json.load(f)

        image_path = os.path.join(image_folder, data['imagePath'])
        image = cv2.imread(image_path)
        if image is None:
            print(f"Image not found or unreadable: {image_path}")
            continue

        height, width = image.shape[:2]
        yolo_labels = []

        for shape in data['shapes']:
            label = shape['label']
            if label not in CLASS_MAP:
                print(f"Unknown label: {label} in {filename}")
                continue
            class_id = CLASS_MAP[label]

            points = shape['points']
            xs = [p[0] for p in points]
            ys = [p[1] for p in points]

            x_min, x_max = min(xs), max(xs)
            y_min, y_max = min(ys), max(ys)

            x_center = (x_min + x_max) / 2.0 / width
            y_center = (y_min + y_max) / 2.0 / height
            bbox_width = (x_max - x_min) / width
            bbox_height = (y_max - y_min) / height

            yolo_labels.append(f"{class_id} {x_center:.6f} {y_center:.6f} {bbox_width:.6f} {bbox_height:.6f}")

        txt_filename = os.path.splitext(filename)[0] + ".txt"
        txt_path = os.path.join(output_folder, txt_filename)
        with open(txt_path, 'w') as f:
            f.write("\n".join(yolo_labels))
        print(f": {filename} → {txt_filename}")

# Example usage
convert_labelme_folder(
    json_folder="./",           
    image_folder="./",           
    output_folder="./labels_yolo"  
)
