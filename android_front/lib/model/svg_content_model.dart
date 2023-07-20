class SvgContent {

  Map<String, String> data;

  SvgContent(this.data);

  String createSVG(Map<String, String> parsedStyle) {

    String r = "";

    if (!data.containsKey("tag")) return r;

    r = "<${data["tag"]}";

    data.forEach((key, value) {
      if (key != "tag" && key != "content" && key != "style") {
        r += " $key=\"$value\" ";
      }
    });

    if (parsedStyle.containsKey(data["tag"])) {
      r += " style=\"${parsedStyle[data["tag"]]}\" ";
    }

    if (!data.containsKey("content")) {
      r += "/>";
    } else {
      r += ">${data["content"]}</${data["tag"]}>";
    }
    return r;
  }

  Map<String, String> parseStyle() {

    Map<String, String> parsed = {};

    if (!data.containsKey("content")) return parsed;

    List<String> d = data["content"]!.split('}');

    for(int i = 0; i < d.length; i++) {
      List<String> dd = d[i].split("{");

      if (dd.length > 1) parsed[dd[0]] = dd[1];
    }

    return parsed;
  }

}