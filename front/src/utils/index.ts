import { AccountIndentity } from "@/dtos";

export function deriv(accNum: number, sAccNum: number): string {
  return `${accNum}/${sAccNum}`;
}

export function b64EncodeUnicode(str: string) {
  return btoa(
    encodeURIComponent(str).replace(/%([0-9A-F]{2})/g, function(_match, p1) {
      return String.fromCharCode(parseInt(p1, 16));
    }),
  );
}

// Decoding base64 ⇢ UTF-8
export function b64DecodeUnicode(str: string) {
  return decodeURIComponent(
    Array.prototype.map
      .call(atob(str), function(c) {
        return "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2);
      })
      .join(""),
  );
}

export function derivBase64({
  account_number: accNum,
  sub_account_number: sAccNum,
}: AccountIndentity): string {
  return b64EncodeUnicode(deriv(accNum, sAccNum));
}

export function getDeriv(accNum: number, sAccNum: number): string {
  return b64EncodeUnicode(deriv(accNum, sAccNum));
}

export function convertBtcToSats(btcAmount: number): number {
  // Convert the amount to a string to avoid floating point arithmetic issues
  const btcAmountStr = btcAmount.toString();

  // Split the string into whole and fractional parts
  const [whole, fractional] = btcAmountStr.split('.');

  // Convert whole part directly to BigInt and multiply by 100 million
  let sats = whole + '00000000';
  let nsats = Number(sats);

  // If there is a fractional part, handle it separately
  if (fractional) {
    // Pad the fractional part to ensure correct calculation
    const paddedFractional = fractional.padEnd(8, '0');
    nsats += Number(paddedFractional)
  }

  return nsats;
}

