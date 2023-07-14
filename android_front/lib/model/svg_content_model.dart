class SvgContent {

  Map<String, String> data;

  SvgContent(this.data);

  String createSVG() {

    String r = "";

    if (!data.containsKey("tag")) return r;

    r = "<${data["tag"]}";

    data.forEach((key, value) {
      if (key != "tag" && key != "content") {
        r += " $key=\"$value\" ";
      }
    });

    if (!data.containsKey("content")) {
      r += "/>";
    } else {
      r += ">${data["content"]}</${data["tag"]}>";
    }
    return r;
  }
}