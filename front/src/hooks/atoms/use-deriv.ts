import { b64DecodeUnicode } from "@/utils";
import { useRouter } from "next/router";

export const useDeriv = () => {
  const router = useRouter();
  let deriv = "";

  try {
    const rawDeriv = router.query.deriv;
    if (typeof rawDeriv === 'string') {
      deriv = b64DecodeUnicode(rawDeriv);
    } else {
      console.error('Invalid or missing "deriv" query parameter');
    }
  } catch (error) {
    console.error('Error decoding "deriv":', error);
  }
  return { deriv }
}
