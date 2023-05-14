import { pokemon } from "./pokemon";

export interface fusedpokemon extends pokemon{
    body: string;
    head: string;
    bodyID: number;
    headID: number;
    headGameID: number;
    bodyGameID: number;
    imgName: string;
}