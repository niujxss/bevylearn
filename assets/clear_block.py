from PIL import Image
import os

def remove_white_background(input_path, output_path, threshold=220):
    """去除图片中的白色背景"""
    img = Image.open(input_path)
    
    # 转换为RGBA（支持透明度）
    img = img.convert("RGBA")
    
    # 获取像素数据
    datas = img.getdata()
    
    # 创建新数据，将白色像素设为透明
    new_data = []
    for item in datas:
        # 如果像素接近白色（RGB值都大于阈值），设为透明
        if item[0] > threshold and item[1] > threshold and item[2] > threshold:
            new_data.append((255, 255, 255, 0))  # 完全透明
        else:
            new_data.append(item)  # 保持原样
    
    # 更新图片数据
    img.putdata(new_data)
    img.save(output_path, "PNG")
    print(f"✅ 已处理: {input_path} -> {output_path}")

# 处理您的坦克精灵表
remove_white_background("1.png", "2.png")