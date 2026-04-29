function getDayType(day) {
  let ret = "";
  switch (day.toLowerCase()) {
    case 'mon':
    case 'tue':
    case 'wed':
    case 'thu':
    case 'fri':
      ret = 'weekday';
      break;
    case 'sat':
    case 'sun':
      ret = 'weekend';
      break;
    default:
      ret = null;
      break;
  }
  return ret;
}
