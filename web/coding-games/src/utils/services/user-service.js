import {ApiInstance} from "./api-instance";

const signIn = async (userSignIn) => {
    return (await ApiInstance.getInstance.post('/user/login', userSignIn));
}

const signUp = async (userSignUp) => {
    return await ApiInstance.getInstance.post('/user', userSignUp);
}

const findOtherPlayers = async (identifier) => {
    return await ApiInstance.getInstance.get(`/user/other_players/${identifier}`)
}

export const UserService = {
    signIn: signIn,
    findPlayers: findOtherPlayers,
    signUp: signUp
}