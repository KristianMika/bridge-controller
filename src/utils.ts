const shortenHexPubkey = (pubkey: string, fstLastCharCount: number): string => {
  return (
    pubkey.slice(0, fstLastCharCount) + "..." + pubkey.slice(-fstLastCharCount)
  );
};

export default shortenHexPubkey;
