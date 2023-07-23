import {CodeMirror} from "@solid-codemirror/codemirror";
import {basicSetup} from "codemirror";
import {java} from "@codemirror/lang-java";
import {python} from "@codemirror/lang-python";
import {
    Box, Button,
    Container,
    Divider, Drawer, FormControl, FormGroup,
    Grid, InputLabel,
    List,
    ListItem,
    ListItemButton,
    ListItemIcon,
    ListItemText, MenuItem, Select, TextField, Typography
} from "@suid/material";
import {oneDark} from "@codemirror/theme-one-dark";
import {createEffect, createSignal, onCleanup, onMount} from "solid-js";
import {DeleteOutlineOutlined, SaveAltOutlined, VideogameAssetOutlined} from "@suid/icons-material";
import {GamesService} from "../utils/services/game-manager-service";
import {indentWithTab} from "@codemirror/commands";
import {keymap} from "@codemirror/view";
import {UserStore} from "../utils/user-store";

export default function Editor() {
    const [codes, setCodes] = createSignal([])
    const [currentCode, setCurrentCode] = createSignal({
        name: "",
        code: "",
        description: "",
        min_players: 2,
        max_players: 2,
        id: null
    })

    /** TODO change the call for token use **/
    onMount(async () => {
            let res = await GamesService.findMyGames();
            setCodes(res.data)
    });


    const [drawerState, setDrawerState] = createSignal(false);
    const [language, setLanguage] = createSignal(python())
    const [languageString, setLanguageString] = createSignal("python")

    const toggleDrawer = (anchor, open) => async (event) => {
        if (event.type === "keydown") {
            const keyboardEvent = event;
            if (keyboardEvent.key === "Tab" || keyboardEvent.key === "Shift")
                return;
        }
        setDrawerState(open);
    };

    const list = (anchor) => (
        <Box
            role="presentation"
            onClick={toggleDrawer(anchor, false)}
            onKeyDown={toggleDrawer(anchor, false)}
            sx={{}}
        >
            <List>
                {["Save", "Delete"].map((text) => (
                    <ListItem disablePadding>
                        <ListItemButton onclick={() => clickDrawerItem(text)}>
                            <ListItemIcon sx={{color: 'white'}}>
                                {text === "Save" ? <SaveAltOutlined></SaveAltOutlined> :
                                    <DeleteOutlineOutlined></DeleteOutlineOutlined>}
                            </ListItemIcon>
                            <ListItemText primary={text}/>
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>
            <Divider/>
            <Typography sx={{textAlign: 'center'}}>Game list</Typography>
            <List>
                {codes().map(game => game.name).map((text) => (
                    <ListItem disablePadding>
                        <ListItemButton onclick={() => clickDrawerGameItem(text)}>
                            <ListItemIcon sx={{color: 'white'}}>
                                <VideogameAssetOutlined></VideogameAssetOutlined>
                            </ListItemIcon>
                            <ListItemText primary={text}/>
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>
        </Box>
    );
    const clickDrawerItem = async (e) => {
        switch (e) {
            case "Save":
                await save()
                break;
            case "Delete":
                await GamesService.remove(currentCode().id)
                break;
        }
    }
    const clickDrawerGameItem = (e) => {
        setCurrentCode(codes().find(item => item.name === e))
    }
    const swapLanguage = (language) => {
        switch (language) {
            case 'python':
                setLanguage(python())
                setLanguageString("python")
                break;
            case 'java':
                setLanguage(java())
                setLanguageString("java")
                break;
            default:
                alert("ton langage n'existe pas")
        }
    }
    const checkPlayersAmount = (e) => {
        if (e.target.value < currentCode().min_players) {
            return alert("tu te fous de ma gueule")
        }
        updateCode("max_players", e.target.value)
    }
    const saveOnCtrlS = async (e) => {
        if (e.ctrlKey && e.key === "s") {
            e.preventDefault()
            await save()
        }
    }

    const save = async () => {
        if (currentCode().id) {
            let payload = createPayload(currentCode())
            await GamesService.updateGame(currentCode().id, payload)
        } else {
            let payload = createPayload(currentCode())
            let res = await GamesService.newGame(payload).data
            console.log(res)
            //updateCode("id", res.id)
        }
    }

    const createPayload = (payload) => {
        console.log(payload)
        payload.language = languageString()
        delete payload.id
        return payload
    }

    const updateCode = (key, value) => {
        let current = currentCode()
        current[key] = value
        setCurrentCode({...current})
    }
    return (
        <>
            <Drawer
                PaperProps={{
                    sx: {
                        backgroundColor: "s282c34",
                        color: "white"
                    }
                }}
                anchor={"right"}
                open={drawerState()}
                sx={{zIndex: 9999}}
                onClose={toggleDrawer("right", false)}
            >
                {list("right")}
            </Drawer>
            <Container maxWidth="sm" sx={{paddingTop: '50px'}}>
                <Button onClick={toggleDrawer("right", true)}>{"Contextual menu"}</Button>
                <Box sx={{flexGrow: 1}}>
                    <Grid container spacing={3}>
                        <Grid item xs={6} md={8}>
                            <CodeMirror value={currentCode().code} onValueChange={(newCode) => updateCode("code", newCode)}
                                        extensions={[basicSetup, language(), keymap.of([indentWithTab])]}
                                        theme={oneDark} onKeyDown={saveOnCtrlS}
                            />
                        </Grid>
                        <Divider></Divider>
                        <Grid item xs={6} md={4}>
                            <FormGroup>
                                <FormControl>
                                    <TextField
                                        required
                                        id="outlined-required"
                                        label="Game name"
                                        value={currentCode().name}
                                        defaultValue={"Game name"}
                                        onChange={(e) => updateCode("name", e.target.value)}
                                        inputProps={{style: {color: "white", justifyContent: "center"}}}
                                        InputLabelProps={{
                                            style: { color: "white" }
                                        }}
                                    />
                                </FormControl>
                                <FormControl
                                    sx={{
                                        m: 1,
                                        minWidth: 120,
                                    }}
                                >
                                    <InputLabel id="demo-simple-select-disabled-label">Program Language</InputLabel>
                                    <Select
                                        labelId="demo-simple-select-disabled-label"
                                        id="demo-simple-select-disabled"
                                        value={languageString()}
                                        onChange={(e) => swapLanguage(e.target.value)}
                                        label="Language"
                                        sx={{color: "white", justifyContent: "center"}}
                                    >
                                        <MenuItem value={'python'}>python</MenuItem>
                                        <MenuItem value={'java'}>java</MenuItem>
                                    </Select>
                                </FormControl>
                                <FormControl
                                    sx={{
                                        m: 1,
                                        minWidth: 120,
                                    }}
                                >
                                    <InputLabel id="demo-simple-select-disabled-label">Minimum players</InputLabel>
                                    <Select
                                        labelId="demo-simple-select-disabled-label"
                                        id="demo-simple-select-disabled"
                                        value={currentCode().min_players}
                                        onChange={(e) => {
                                            updateCode("min_players", e.target.value)
                                        }}
                                        label="Minimum players"
                                        sx={{color: "white", justifyContent: "center"}}
                                    >
                                        <MenuItem value={2}>2</MenuItem>
                                        <MenuItem value={3}>3</MenuItem>
                                        <MenuItem value={4}>4</MenuItem>
                                        <MenuItem value={5}>5</MenuItem>
                                        <MenuItem value={6}>6</MenuItem>
                                        <MenuItem value={7}>7</MenuItem>
                                        <MenuItem value={8}>8</MenuItem>
                                    </Select>
                                </FormControl>
                                <FormControl sx={{
                                    m: 1,
                                    minWidth: 120,
                                }}>
                                    <InputLabel id="demo-simple-select-disabled-label">Maximum players</InputLabel>
                                    <Select
                                        labelId="demo-simple-select-disabled-label"
                                        id="demo-simple-select-disabled"
                                        value={currentCode().max_players}
                                        label="Maximum players"
                                        sx={{color: "white", justifyContent: "center"}}
                                        onChange={checkPlayersAmount}
                                    >
                                        <MenuItem value={2}>2</MenuItem>
                                        <MenuItem value={3}>3</MenuItem>
                                        <MenuItem value={4}>4</MenuItem>
                                        <MenuItem value={5}>5</MenuItem>
                                        <MenuItem value={6}>6</MenuItem>
                                        <MenuItem value={7}>7</MenuItem>
                                        <MenuItem value={8}>8</MenuItem>
                                    </Select>
                                </FormControl>
                                <FormControl>
                                    <TextField
                                        id="filled-multiline-flexible"
                                        label="Game Description"
                                        multiline
                                        maxRows={4}
                                        value={currentCode().description}
                                        variant="filled"
                                        onChange={(e) => {
                                            updateCode("description", e.target.value)
                                        }}
                                        inputProps={{style: {color: "white", justifyContent: "center"}}}
                                        InputLabelProps={{
                                            style: { color: "white" }
                                        }}
                                    />
                                </FormControl>
                            </FormGroup>
                        </Grid>
                    </Grid>
                </Box>
            </Container>
        </>
    )
}