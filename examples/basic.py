from frames_viewer import Viewer
import numpy as np
import time

def main():
    fv = Viewer()
    fv.start()  # pops the viewer window in a separate thread

    frame1 = np.eye(4, dtype=np.float32)
    frame1[:3, 3] = [0.0, 0.1, 0.1]  # meters

    try:
        while True:
            frame1[:3, 3][0] = 0.1 * np.sin(2 * np.pi * 1.0 * time.time())
            fv.push_frame(frame1, "frame1")
            time.sleep(0.01)
    except KeyboardInterrupt:
        fv.stop()

if __name__ == "__main__":
    main() 