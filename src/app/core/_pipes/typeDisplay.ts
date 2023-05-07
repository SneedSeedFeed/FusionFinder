import { Pipe, PipeTransform } from '@angular/core';

//Converts the types into a slightly clearer format for display
@Pipe({name: 'typeDisplay', pure: true})
export class typeDisplay implements PipeTransform {
    transform(types: string[]): string {
        if(types.length==1){
            return types[0]
        }else if (types[0] == types[1]){
            return types[0]
        }

        return types[0] + " / " + types[1]
    }
  }