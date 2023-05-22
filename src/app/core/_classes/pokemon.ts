import { ability } from "../_interfaces/ability";

export class pokemon {
    //For non gen 1-2, this is a dictionary/map of natdex to infinite fusion dex numbers
    static readonly IDList: { [natdex: number]: number } = {
        298: 252, 360: 253, 424: 254, 429: 255, 430: 256, 438: 257, 439: 258, 440: 259,
        446: 260, 458: 261, 461: 262, 462: 263, 463: 264, 464: 265, 465: 266, 466: 267, 467: 268, 468: 269,
        469: 270, 470: 271, 471: 272, 472: 273, 473: 274, 474: 275, 252: 276, 253: 277, 254: 278, 255: 279,
        256: 280, 257: 281, 258: 282, 259: 283, 260: 284, 280: 285, 281: 286, 282: 287, 475: 288, 292: 289,
        352: 290, 374: 291, 375: 292, 376: 293, 399: 294, 442: 295, 448: 296, 443: 297, 444: 298, 445: 299,
        303: 300, 345: 301, 346: 302, 347: 303, 348: 304, 408: 305, 409: 306, 410: 307, 411: 308, 289: 309,
        359: 310, 355: 311, 356: 312, 477: 313, 321: 314, 493: 315, 387: 316, 388: 317, 389: 318, 390: 319,
        391: 320, 392: 321, 393: 322, 394: 323, 395: 324, 299: 325, 476: 326, 679: 327, 680: 328, 681: 329,
        624: 330, 625: 331, 405: 332, 306: 333, 330: 334, 350: 335, 373: 336, 601: 337, 571: 338, 700: 339,
        382: 340, 383: 341, 384: 342, 483: 343, 484: 344, 487: 345, 486: 346, 491: 347, 649: 348, 643: 349,
        644: 350, 646: 351, 407: 352, 426: 353, 428: 354, 286: 355, 291: 356, 354: 357, 479: 358, 579: 359,
        547: 360, 553: 361, 563: 362, 596: 363, 598: 364, 607: 365, 608: 366, 609: 367, 612: 368, 623: 369,
        771: 370, 707: 371, 663: 372, 778: 373, 637: 374, 633: 375, 634: 376, 635: 377, 380: 378, 381: 379,
        386: 380, 385: 381, 290: 382, 400: 383, 447: 384, 287: 385, 288: 386, 320: 387, 403: 388, 404: 389,
        304: 390, 305: 391, 328: 392, 329: 393, 349: 394, 371: 395, 372: 396, 599: 397, 600: 398, 570: 399,
        406: 400, 315: 401, 425: 402, 427: 403, 285: 404, 353: 405, 577: 406, 578: 407, 546: 408, 551: 409,
        552: 410, 562: 411, 595: 412, 597: 413, 610: 414, 611: 415, 622: 416, 661: 417, 662: 418, 636: 419,
        618: 420
    }


    id: number;
    name: string;
    type: string[];
    HP: number;
    attack: number;
    defense: number;
    spAttack: number;
    spDefense: number;
    speed: number;
    abilities: ability[];
    newdexID: number;
    BST: number;
    [key: string]: any;

    constructor(id: number, name: string, type: string[], HP: number, attack: number, defense: number, spAttack: number, spDefense: number, speed: number, abilities: ability[]) {
        this.id = id
        this.name = name
        this.type = []
        type.forEach(val => {
            this.type.push(val.toLowerCase())
        })
        this.HP = HP
        this.attack = attack
        this.defense = defense
        this.spAttack = spAttack
        this.spDefense = spDefense
        this.speed = speed
        this.abilities = abilities
        this.newdexID = pokemon.getNewDexID(id)
        this.BST = (HP + attack + defense + spAttack + spDefense + speed)
    }

    private static getNewDexID(natDexID: number): number {
        if (natDexID <= 251) {
            return natDexID
        }
        if (pokemon.IDList[natDexID]) {
            return pokemon.IDList[natDexID]
        }

        return natDexID
    }
}