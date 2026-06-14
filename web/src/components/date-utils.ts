import {
  addDays,
  addHours,
  addWeeks,
  differenceInDays,
  differenceInHours,
  differenceInMinutes,
  differenceInWeeks,
} from "date-fns";

/**
 * Combines a count and a name of an item into a plural form
 * @param name The name of the item to display
 * @param count The count
 * @returns The pluralised form of the item, or singular if count === 1
 */
const pluralise = (
  name: string,
  count: number,
): `${number} ${string}${"s" | ""}` => {
  if (count === 1) {
    return `${count} ${name}`;
  }

  return `${count} ${name}s`;
};

export const getExpiresAtOptions = (): Date[] => {
  const now = new Date();

  return [
    addHours(now, 1),
    addHours(now, 3),
    addHours(now, 6),
    addDays(now, 1),
    addDays(now, 3),
    addWeeks(now, 1),
  ];
};

export const getRemainingTimeText = (expiresAt: Date): string => {
  const now = new Date();
  const hours = differenceInHours(expiresAt, now);

  if (hours === 0) {
    const minutes = differenceInMinutes(expiresAt, now);
    return pluralise("minute", minutes);
  }

  if (hours < 72) {
    return pluralise("hour", hours);
  }

  const days = differenceInDays(expiresAt, now);

  if (days < 7) {
    return pluralise("day", days);
  }

  const weeks = differenceInWeeks(expiresAt, now);
  return pluralise("week", weeks);
};
