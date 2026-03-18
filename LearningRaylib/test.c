#include "raylib.h"

int main() {
    InitWindow(800, 600, "Raylib Test");

    while (!WindowShouldClose()) {
        BeginDrawing();
        ClearBackground(RAYWHITE);
        DrawText("Raylib works!", 300, 300, 20, BLACK);
        EndDrawing();
    }

    CloseWindow();
}