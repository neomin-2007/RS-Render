# RS-Render

This project is a renderer that converts three-dimensional (3D) objects represented in .json files into a two-dimensional (2D) projection. The renderer uses Rust and the macroquad library to draw the edges of the 3D object on a 2D canvas. It also includes interactive features such as rotation, translation, and zooming.

![image](https://i.imgur.com/mMkkhR0.png)

## Features

- **JSON File Parsing**: The renderer accepts `.json` files containing the definition of vertices and edges of a 3D object.
- **3D to 2D Projection**: Uses a simple projection to convert 3D coordinates into 2D.
- **Rotation and Movement**: The renderer allows you to rotate and move the object.
- **Real-Time Rendering**: Draws the edges of the 3D object in a graphical window.

# Project Structure

This project consists of two main classes:

1. **`RenderPanel`**: Responsible for rendering the 3D object onto a 2D screen.
    - **Method `project`**: Converts 3D coordinates into 2D.
    - **Method `paintComponent`**: Draws the edges of the object on the screen.

2. **`Geometry`**: Represents the geometry of the 3D object.
    - **Attributes**:
        - `vertex`: A matrix of vertices (3D coordinates).
        - `edges`: A matrix of edges (connections between vertices).
        - `user_angle_x`: The rotation of axis X.
        - `user_angle_y`: The rotation of axis Y.
        - `user_distance`: The distance of perspective.
        - `user_x` and `user_y`: The coordinates X and Y.

## Archive Example

The `.json` archive example:

```json
{
  "vertex": [
      [0.0, 100.0, 0.0],
      [100.0, 100.0, 0.0],
      [100.0, 0.0, 0.0],
      [0.0, 0.0, 0.0],
      [0.0, 100.0, 100.0],
      [100.0, 100.0, 100.0],
      [100.0, 0.0, 100.0],
      [0.0, 0.0, 100.0]
  ],
  "edges": [
      [0, 1], [1, 2], [2, 3], [3, 0],
      [7, 6], [6, 5], [5, 4], [4, 7],
      [0, 4], [1, 5], [2, 6], [3, 7]
  ],
  "user_angle_x": 0.0,
  "user_angle_y": 0.0,
  "user_scale": 500.0,
  "user_distance": 500.0,
  "user_x": 0.0,
  "user_y": 0.0
}
```
