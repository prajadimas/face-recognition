# Face Recognition with Rust

Still on progress ...

## Table of Contents

- [Model Lists](#models)
- [Flow](#flow)
- [Preparations](#preparations)
  - [OpenCV](#opencv)
  - [Tensorflow](#tensorflow)
- [Tests](#tests)
- [Benchmark](#benchmark)

## Models

List of _Pretrained Models_ found:
- Haarcascade (Source on Rust, [Face Detection Example OpenCV Rust](https://github.com/twistedfall/opencv-rust/blob/master/examples/video_facedetect.rs))
- MTCNN (Source on Rust, [Face Detection with Tensorflow Rust](https://cetra3.github.io/blog/face-detection-with-tensorflow-rust/))
- Seetaface (Source on C++, [SeetaFace Engine](https://github.com/seetaface/SeetaFaceEngine))
- Face Recognition (Python Library, [ageitgey/face_recognition: The world's simplest facial recognition api for Python and the command line](https://github.com/ageitgey/face_recognition))
- Facenet Keras (Source on Python, [How to Develop a Face Recognition System Using FaceNet in Keras](https://machinelearningmastery.com/how-to-develop-a-face-recognition-system-using-facenet-in-keras-and-an-svm-classifier/))
- BlazeFace (Source on Javascript, [MediaPipe Face Detection](https://google.github.io/mediapipe/solutions/face_detection.html))

## Flow

1. Face Detection then crop images to prepare face models
2. Face Detection using list of models
3. Face Recognition to differentiate faces

## Preparations

### OpenCV:

Steps to reproduce on Linux - [OpenCV: Installation in Linux](https://docs.opencv.org/master/d7/d9f/tutorial_linux_install.html) with some adjustment:
1. Install dependencies (cmake, g++/gcc, wget, unzip)
2. Install OpenCV (3.2, 3.4, 4.x)
  - Manual
    1. Download from OpenCV Repo - Latest 4.5.3 (```wget -O opencv.zip https://github.com/opencv/opencv/archive/master.zip```)
    2. Unpack downloaded OpenCV (```unzip opencv.zip```)
    3. Create build directory (```mv opencv-master opencv && mkdir -p build && cd build```)
    4. Pre-Configure (```rm ../opencv/CMakeCache.txt```)
    5. Configure with libgtk2.0 disable qt (```cmake -D WITH_QT=OFF ../opencv```)
    6. Build (```make```), you can run compilation processes in parallel by using (```make -j4```)
    7. Run installation, you need elevated privileges (```sudo make install```)
    8. Post-Configure (```export OpenCV_DIR=<*.cmake location> && export LD_LIBRARY_PATH=<*.so location>```)
  - [Prebuilt version](https://docs.opencv.org/master/d0/d3d/tutorial_general_install.html#tutorial_general_install_prebuilt)
3. Run Apps (```cargo run```)

### Tensorflow:

Read this [Prerequisites](https://github.com/tensorflow/rust#prerequisites):

## Tests

Test face detection on [sample image](tests/img) using lists of Models, Running with:

```console
foo@bar:~$ cargo run --example <models> <input_file> <output_file>
```

- Using Haarcascade:

![Alt text](output/sample01_haarcascade.jpg?raw=true "Sample-01 Haarcascade")
![Alt text](output/sample02_haarcascade.jpg?raw=true "Sample-02 Haarcascade")

- Using MTCNN:

![Alt text](output/sample01_mtcnn.jpg?raw=true "Sample-01 MTCNN")
![Alt text](output/sample02_mtcnn.jpg?raw=true "Sample-02 MTCNN")

- Using Seetaface:

![Alt text](output/sample01_seetaface.jpg?raw=true "Sample-01 SeetaFace")
![Alt text](output/sample02_seetaface.jpg?raw=true "Sample-02 SeetaFace")

## Benchmark

On progress
