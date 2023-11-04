/**
 * Shortens a long hex string into form `inputString[:nChars]...inputString[-nChars:]`
 * @param inputString string to be shortened
 * @param nChars the number of characters to be shown on each end of the string
 * @returns shortened string
 */
const shortenHexString = (inputString: string, nChars: number): string => {
  return inputString.slice(0, nChars) + "..." + inputString.slice(-nChars);
};

export default shortenHexString;
