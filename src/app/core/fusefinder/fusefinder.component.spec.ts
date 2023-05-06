import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FusefinderComponent } from './fusefinder.component';

describe('FusefinderComponent', () => {
  let component: FusefinderComponent;
  let fixture: ComponentFixture<FusefinderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ FusefinderComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FusefinderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
