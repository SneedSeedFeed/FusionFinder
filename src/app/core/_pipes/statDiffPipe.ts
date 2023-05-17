import { Pipe, PipeTransform } from '@angular/core';

@Pipe({ name: 'statDiffDisplay', pure: true })
export class statDiffDisplay implements PipeTransform {
    transform(value: number, comparedstat: number): string {
        let result = value - comparedstat
        if (result > 0) { return "+" + result }
        if (result < 0) { return result.toString() }
        return "0"
    }
}