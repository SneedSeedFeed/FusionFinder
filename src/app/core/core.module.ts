//Angular Imports
import { Input, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { BrowserModule } from "@angular/platform-browser";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";

//My Imports
import { FusefinderComponent } from './fusefinder/fusefinder.component';
import { CoreRoutingModule } from './core-routing.module';
import { PageContainerComponent } from './page-container/page-container.component';
import { typeDisplay } from './_pipes/typeDisplay';
import {abilityDisplay} from './_pipes/abilityDisplay'

//PrimeNG Imports
import { CardModule } from 'primeng/card';
import { VirtualScrollerModule } from 'primeng/virtualscroller';
import { DropdownModule } from 'primeng/dropdown';
import { DividerModule } from 'primeng/divider';
import { InputText, InputTextModule } from 'primeng/inputtext'


@NgModule({
  declarations: [
    FusefinderComponent,
    PageContainerComponent,
    typeDisplay,
    abilityDisplay
  ],
  imports: [
    BrowserModule,
    InputTextModule,
    BrowserAnimationsModule,
    CommonModule,
    CoreRoutingModule,
    CardModule,
    VirtualScrollerModule,
    DropdownModule,
    FormsModule,
    DividerModule
  ],
  exports: [PageContainerComponent]
})
export class CoreModule { }
