import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FusefinderComponent } from './fusefinder/fusefinder.component';
import { CoreRoutingModule } from './core-routing.module';
import { PageContainerComponent } from './page-container/page-container.component';



@NgModule({
  declarations: [
    FusefinderComponent,
    PageContainerComponent
  ],
  imports: [
    CommonModule,
    CoreRoutingModule
  ],
  exports: [PageContainerComponent]
})
export class CoreModule { }
