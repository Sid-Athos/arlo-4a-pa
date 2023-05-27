
class Game {

  int id;
  String name;
  String? description;
  int minPlayers;
  int maxPlayers;

  Game({
    required this.id,
    required this.name,
    required this.description,
    required this.minPlayers,
    required this.maxPlayers
  });

}