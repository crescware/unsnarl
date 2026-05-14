class D {
  m() {
    return 1;
  }
}
class C extends D {
  m() {
    return super.m();
  }
}
