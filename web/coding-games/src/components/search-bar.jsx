import SearchIcon from "@suid/icons-material/Search";
import {Box, TextField} from "@suid/material";
import {createSignal} from "solid-js";
import {UserService} from "../utils/services/user-service";

export default function SearchBar({setUserList}){
    const [search, setSearch] = createSignal("")
    const test = async (e) => {
        if (e.key === "Enter") {
            let res = await UserService.searchPlayer(search())
            setUserList(res.data)
        }
        setSearch(e.target.value)
    }
    return (<>
            <Box >
                <SearchIcon sx={{color:"white"}}/>
                <TextField id="input-with-sx" variant="standard" onKeyDown={test} inputProps={{ style: { color: "white", justifyContent:"center", borderColor:"white" } }}/>
            </Box>
        </>
    )
}