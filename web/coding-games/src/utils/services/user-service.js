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

const findUser = async (identifier) => {
    return await ApiInstance.getInstance.get(`/user/search/${identifier}`)
}

const addFriend = async (user) => {
    return await ApiInstance.getInstance.post(`/friend-list`, user)
}

const getFriendRequests = async () => {
    return await ApiInstance.getInstance.get(`/friend-list/requests`)
}

const acceptFriendRequest = async (id) => {
    return await ApiInstance.getInstance.put(`/friend-list/${id}`)
}

const declineFriendRequest = async (id) => {
    return await ApiInstance.getInstance.delete(`/friend-list/${id}`)
}


export const UserService = {
    signIn: signIn,
    findPlayers: findOtherPlayers,
    signUp: signUp,
    searchPlayer: findUser,
    addFriend: addFriend,
    friendRequests: getFriendRequests,
    acceptFriend: acceptFriendRequest,
    declineFriend: declineFriendRequest
}