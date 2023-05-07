import { Pipe, PipeTransform } from '@angular/core';
import { ability } from '../_interfaces/ability';

//Converts the abilities into a displayable format
@Pipe({name: 'abilityDisplay', pure: true})
export class abilityDisplay implements PipeTransform {
    transform(abilities: ability[]): string {
        let toShow: string = ""

        abilities.forEach(val => {
            if(val.isHidden == true){
                toShow += "("+val.name+")"
            } else{
                toShow += val.name
            }
            toShow += " | "
        })

        return toShow
    }
  }