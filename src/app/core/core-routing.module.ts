import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, Routes } from '@angular/router';
import { FusefinderComponent } from './fusefinder/fusefinder.component';

const routes: Routes = [{path: '', component: FusefinderComponent}]

@NgModule({
  declarations: [],
  imports: [
    CommonModule,
    RouterModule.forRoot(routes),
  ],
  exports: [RouterModule]
})
export class CoreRoutingModule { }
