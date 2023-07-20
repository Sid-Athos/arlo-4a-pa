import { CodeMirror } from "@solid-codemirror/codemirror";
import { basicSetup } from "codemirror";
import { java } from "@codemirror/lang-java";
import {Box, Container, Grid} from "@suid/material";
import { oneDark } from "@codemirror/theme-one-dark";
import {Show} from "solid-js";

export default function Editor(){
    const code = 'import json\n' +
        'PLAYER_ARG = {\n' +
        '\t"name" : "players",\n' +
        '\t"type" : "int",\n' +
        '\t"min" : 2,\n' +
        '\t"max" : 2,\n' +
        '\t"description" : "number of players"\n' +
        '}\n' +
        '\n' +
        '\n' +
        '\n' +
        'def read_json():\n' +
        '\tcontent = input()\n' +
        '\topening_curly_brackets = content.count("{")\n' +
        '\tclosing_curly_brackets = content.count("}")\n' +
        '\twhile opening_curly_brackets < 1 or closing_curly_brackets != opening_curly_brackets:\n' +
        '\t\tcontent += input()\n' +
        '\t\topening_curly_brackets = content.count("{")\n' +
        '\t\tclosing_curly_brackets = content.count("}")\n' +
        '\tcontent = content.strip()\n' +
        '\tcontent = json.loads(content)\n' +
        '\treturn content\n' +
        '\n' +
        'def print_error(type, **kwargs):\n' +
        '\tquit = kwargs.pop("fatal", False)\n' +
        '\td = {"type":type}\n' +
        '\td.update(kwargs)\n' +
        '\tprint(json.dumps({"errors":[d]}, indent=2))\n' +
        '\tif quit:\n' +
        '\t\texit()\n' +
        '\n' +
        '\n' +
        '\n' +
        'def init(grid):\n' +
        '\tcontent = read_json()\n' +
        '\tif "init" not in content:\n' +
        '\t\tprint_error("BAD_FORMAT", \n' +
        '\t\t\tmessage="init key is missing", \n' +
        '\t\t\tfatal=True)\n' +
        '\tinit_content = content["init"]\n' +
        '\tif "players" not in init_content : \n' +
        '\t\tprint_error("MISSING_ARGUMENT", \n' +
        '\t\t\targ=PLAYER_ARG, \n' +
        '\t\t\tfatal=True)\n' +
        '\tif init_content["players"] != 2:\n' +
        '\t\tprint_error("INCORRECT_VALUE", \n' +
        '\t\t\targ=PLAYER_ARG, \n' +
        '\t\t\tvalue=init_content["players"],\n' +
        '\t\t\tfatal=True)\n' +
        '\tif len(init_content)>1:\n' +
        '\t\tfor k,v in init_content.items():\n' +
        '\t\t\tif k != "players":\n' +
        '\t\t\t\tprint_error("UNEXPECTED_ARGUMENT", \n' +
        '\t\t\t\t\targname=k, \n' +
        '\t\t\t\t\tvalue=v,\n' +
        '\t\t\t\t\tfatal=True)\n' +
        '\n' +
        '\tprint(json.dumps(grid.current_instructions, indent=2))\n' +
        '\n' +
        '\n' +
        '\n' +
        'def turn(grid):\n' +
        '\tcontent = read_json()\n' +
        '\tif "actions" not in content:\n' +
        '\t\tprint_error("BAD_FORMAT", \n' +
        '\t\t\tmessage="actions key is missing")\n' +
        '\t\treturn False\n' +
        '\tactions = content["actions"]\n' +
        '\tif not isinstance(actions, list):\n' +
        '\t\tprint_error("BAD_FORMAT", \n' +
        '\t\t\tmessage="actions value is not a list")\n' +
        '\t\treturn False\n' +
        '\tif len(actions) != 1:\n' +
        '\t\tprint_error("BAD_FORMAT", \n' +
        '\t\t\tmessage="exactly one action is expected")\n' +
        '\t\treturn False\n' +
        '\taction = actions[0]\n' +
        '\tif not isinstance(action, dict):\n' +
        '\t\tprint_error("BAD_FORMAT", \n' +
        '\t\t\tmessage="action is not a dict containing player, x and y values")\n' +
        '\tif not {"x", "y","player"}.issubset(action.keys()):\n' +
        '\t\tprint_error("BAD_FORMAT", \n' +
        '\t\t\tmessage="action is not a dict containing player, x and y values")\n' +
        '\tif int(action["player"]) != grid.current_player:\n' +
        '\t\tprint_error("MISSING_ACTION", \n' +
        '\t\t\tplayer=grid.current_player,\n' +
        '\t\t\trequested_action=grid.current_action)\t\t\n' +
        '\t\treturn False\n' +
        '\ttry:\n' +
        '\t\tscore = grid.click(int(action["x"]), int(action["y"]))\n' +
        '\texcept AttributeError:\n' +
        '\t\tprint_error("WRONG_ACTION", \n' +
        '\t\t\tsubtype="OUT_OF_ZONE",\n' +
        '\t\t\tplayer=grid.current_player,\n' +
        '\t\t\taction=action,\n' +
        '\t\t\trequested_action=grid.current_action)\n' +
        '\t\treturn False\t\n' +
        '\tprint(json.dumps(grid.current_instructions, indent=2))\n' +
        '\n' +
        '\treturn score != 0\n' +
        '\n' +
        '\n' +
        '\n' +
        'def str_all_values(content):\n' +
        '\tif isinstance(content, dict):\n' +
        '\t\tfor k,v in content.items():\n' +
        '\t\t\tcontent[k] = str_all_values(v)\n' +
        '\n' +
        '\telif isinstance(content, list):\n' +
        '\t\tfor i,v in enumerate(content):\n' +
        '\t\t\tcontent[i] = str_all_values(v)\n' +
        '\telse:\n' +
        '\t\tcontent = str(content)\n' +
        '\treturn content\n' +
        '\n' +
        '\n' +
        'class Grid:\n' +
        '\tdef __init__(self, case_size=100):\n' +
        '\t\tself.__case_size = case_size\n' +
        '\t\tself.__grid = [[0]*3 for x in range(3)]\n' +
        '\t\tself.__current_player = 1\n' +
        '\n' +
        '\tdef check_winner(self):\n' +
        '\t\tgrid = self.__grid\n' +
        '\t\tfor i in range(3):\n' +
        '\t\t\tif grid[0][i] != 0: \n' +
        '\t\t\t\tif all(grid[j][i] == grid[0][i] for j in range(1,3)):\n' +
        '\t\t\t\t\treturn grid[0][i]\n' +
        '\t\t\tif grid[i][0] != 0: \n' +
        '\t\t\t\tif all(grid[i][j] == grid[i][0] for j in range(1,3)):\n' +
        '\t\t\t\t\treturn grid[i][0] \n' +
        '\t\tif grid[0][0] != 0: \n' +
        '\t\t\tif all(grid[j][j] == grid[0][0] for j in range(1,3)):\n' +
        '\t\t\t\treturn grid[0][0]\n' +
        '\t\tif grid[0][2] != 0: \n' +
        '\t\t\tif all(grid[j][2-j] == grid[0][2] for j in range(1,3)):\n' +
        '\t\t\t\treturn grid[0][2]\n' +
        '\t\treturn 0\n' +
        '\n' +
        '\tdef click(self, x, y):\n' +
        '\t\tx = x//self.__case_size\n' +
        '\t\ty = y//self.__case_size\n' +
        '\t\tif x < 0 or x > 2:\n' +
        '\t\t\traise AttributeError("x not in range")\n' +
        '\t\tif y < 0 or y > 2:\n' +
        '\t\t\traise AttributeError("y not in range")\n' +
        '\t\tif self.__grid[x][y] != 0:\n' +
        '\t\t\traise AttributeError("case already taken")\n' +
        '\t\tself.__grid[x][y] = self.__current_player\n' +
        '\t\tself.__current_player = 1 if self.__current_player == 2 else 2\n' +
        '\t\treturn self.check_winner()\n' +
        '\n' +
        '\n' +
        '\t@property\n' +
        '\tdef clickable_zones(self):\n' +
        '\t\tzones = []\n' +
        '\t\tfor x in range(3):\n' +
        '\t\t\tfor y in range(3):\n' +
        '\t\t\t\tif self.__grid[x][y] == 0:\n' +
        '\t\t\t\t\tzones.append({\n' +
        '\t\t\t\t\t\t"x":x*self.__case_size,\n' +
        '\t\t\t\t\t\t"y":y*self.__case_size,\n' +
        '\t\t\t\t\t\t"width":self.__case_size,\n' +
        '\t\t\t\t\t\t"height":self.__case_size\n' +
        '\t\t\t\t\t\t})\n' +
        '\t\treturn zones\n' +
        '\n' +
        '\n' +
        '\t@property\n' +
        '\tdef current_player(self):\n' +
        '\t\treturn self.__current_player\n' +
        '\t\n' +
        '\n' +
        '\t@property\n' +
        '\tdef current_action(self):\n' +
        '\t\treturn {\n' +
        '\t\t\t"type" : "CLICK",\n' +
        '\t\t\t"player": self.current_player,\n' +
        '\t\t\t"zones" : self.clickable_zones\n' +
        '\t\t}\n' +
        '\n' +
        '\t@property\n' +
        '\tdef current_display(self):\n' +
        '\t\tcontent = [\n' +
        '\t\t\t{"tag":"style", "content": "line{stroke:black;stroke-width:4;}"}\n' +
        '\t\t]\n' +
        '\n' +
        '\t\tfor i in range(1,3):\n' +
        '\t\t\tcontent.append({\n' +
        '\t\t\t\t"tag":"line", \n' +
        '\t\t\t\t"x1":0, \n' +
        '\t\t\t\t"y1":i*self.__case_size,\n' +
        '\t\t\t\t"x2":3*self.__case_size,\n' +
        '\t\t\t\t"y2":i*self.__case_size\n' +
        '\t\t\t\t})\n' +
        '\n' +
        '\t\t\tcontent.append({\n' +
        '\t\t\t\t"tag":"line", \n' +
        '\t\t\t\t"x1":i*self.__case_size,\n' +
        '\t\t\t\t"y1":0, \n' +
        '\t\t\t\t"x2":i*self.__case_size,\n' +
        '\t\t\t\t"y2":3*self.__case_size\n' +
        '\t\t\t\t})\n' +
        '\n' +
        '\t\tfor x in range(3):\n' +
        '\t\t\tfor y in range(3):\n' +
        '\t\t\t\tif self.__grid[x][y] == 0:\n' +
        '\t\t\t\t\tcontinue\n' +
        '\t\t\t\tcolor = "blue"\n' +
        '\t\t\t\tif self.__grid[x][y] == 2:\n' +
        '\t\t\t\t\tcolor = "red"\n' +
        '\t\t\t\tcontent.append({\n' +
        '\t\t\t\t\t"tag":"circle",\n' +
        '\t\t\t\t\t"cx":int((0.5+x)*self.__case_size),\n' +
        '\t\t\t\t\t"cy":int((0.5+y)*self.__case_size),\n' +
        '\t\t\t\t\t"r":self.__case_size//3,\n' +
        '\t\t\t\t\t"fill":color\n' +
        '\t\t\t\t\t})\n' +
        '\t\treturn str_all_values({\n' +
        '\t\t\t"width":self.__case_size*3,\n' +
        '\t\t\t"height":self.__case_size*3,\n' +
        '\t\t\t"content":content\n' +
        '\t\t})\n' +
        '\n' +
        '\t@property\n' +
        '\tdef current_instructions(self):\n' +
        '\t\tdisplay = self.current_display \n' +
        '\t\tscores = [0,0]\n' +
        '\t\tgame_over = False\n' +
        '\t\twinner = self.check_winner()\n' +
        '\t\tif winner:\n' +
        '\t\t\tgame_over = True \n' +
        '\t\t\tscores[winner-1] = 1\n' +
        '\t\treturn {\n' +
        '\t\t\t"displays":[{**display, "player":1},{**display, "player":2}],\n' +
        '\t\t\t"requested_actions":[self.current_action],\n' +
        '\t\t\t"game_state":{\n' +
        '\t\t\t\t"scores": scores,\n' +
        '\t\t\t\t"game_over":game_over\n' +
        '\t\t\t\t}\n' +
        '\t\t\t}\n' +
        '\t\t\t\n' +
        'if __name__ == "__main__":\n' +
        '\tgrid = Grid()\n' +
        '\tinit(grid)\n' +
        '\twhile not turn(grid):\n' +
        '\t\tpass'

    return (
        <>
            <Container maxWidth="sm" sx={{paddingTop: '50px'}}>
            <Box sx={{ flexGrow: 1 }}>
                <Grid container spacing={2}>
                    <Show when={code.length > 0}>
                        <CodeMirror value={code} extensions={[basicSetup, java()]} theme={oneDark} />
                    </Show>
                </Grid>
            </Box>
            </Container>

        </>
    )
}