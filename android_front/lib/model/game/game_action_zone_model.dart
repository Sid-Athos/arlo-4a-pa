class GameActionZone {

  int x;
  int y;
  int width;
  int height;

  GameActionZone({
    required this.x,
    required this.y,
    required this.width,
    required this.height
  });

  bool isInZone(double x, double y) {
    if (x >= this.x && x < this.x + width && y >= this.y && y < this.y + height) {
      return true;
    }
    return false;
  }
}