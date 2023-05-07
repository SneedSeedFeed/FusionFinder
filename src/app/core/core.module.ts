//Angular Imports
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { BrowserModule } from "@angular/platform-browser";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";

//My Imports
import { FusefinderComponent } from './fusefinder/fusefinder.component';
import { CoreRoutingModule } from './core-routing.module';
import { PageContainerComponent } from './page-container/page-container.component';
import { typeDisplay } from './_pipes/typeDisplay';

//PrimeNG Imports
import { CardModule } from 'primeng/card';
import { VirtualScrollerModule } from 'primeng/virtualscroller';
import { DropdownModule } from 'primeng/dropdown';


@NgModule({
  declarations: [
    FusefinderComponent,
    PageContainerComponent,
    typeDisplay
  ],
  imports: [
    BrowserModule,
    BrowserAnimationsModule,
    CommonModule,
    CoreRoutingModule,
    CardModule,
    VirtualScrollerModule,
    DropdownModule,
    FormsModule
  ],
  exports: [PageContainerComponent]
})
export class CoreModule { }
