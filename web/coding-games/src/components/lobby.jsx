import {createSignal} from "solid-js";

const fetchGames = async () =>
    (await fetch(`http://swapi.dev/api/people/${id}/`)).json();

export default function Lobby(){
    const [games,setGames] = createSignal([])

};