//Angular Imports
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

//My Imports
import { FusefinderComponent } from './fusefinder/fusefinder.component';
import { CoreRoutingModule } from './core-routing.module';
import { PageContainerComponent } from './page-container/page-container.component';

//PrimeNG Imports
import { CardModule } from 'primeng/card';
import { VirtualScrollerModule } from 'primeng/virtualscroller';
import { DropdownModule } from 'primeng/dropdown';
import { BrowserModule } from "@angular/platform-browser";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";

@NgModule({
  declarations: [
    FusefinderComponent,
    PageContainerComponent
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
