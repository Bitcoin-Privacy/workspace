import { b64DecodeUnicode } from "@/utils";
import { useRouter } from "next/router";

export const useStatecoinDetail = () => {
  const router = useRouter();
  let deriv = "";
  let statechainId  = router.query.id;

  try {
    const rawDeriv = router.query.deriv;
    
    if (typeof rawDeriv === "string" ) {
      deriv = b64DecodeUnicode(rawDeriv);

    } else {
      console.error('Invalid or missing "deriv" && "statechain_id" query parameter');
    }
  } catch (error) {
    console.error('Error decoding "deriv":', error);
  }
  return { deriv, statechainId};
};
