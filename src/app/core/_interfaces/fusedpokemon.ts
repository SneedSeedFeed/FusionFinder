import { pokemon } from "./pokemon";

export interface fusedpokemon extends pokemon{
    body: string;
    head: string;
    bodyID: number;
    headID: number;
}