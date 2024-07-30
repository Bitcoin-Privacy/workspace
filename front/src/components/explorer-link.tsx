import { FC } from "react";
import { Link } from "@chakra-ui/react";

export enum ExplorerLinkType {
  ADDRESS,
  TRANSACTION,
}

interface ICopier {
  id: string;
  type: ExplorerLinkType;
}

export const ExplorerLink: FC<ICopier> = (props) => {
  const { id, type } = props;
  const link =
    "https://blockstream.info/testnet/" +
    (type === ExplorerLinkType.ADDRESS ? "address/" : "tx/") +
    id;

  return (
    <Link display="block" href={link} rel="noopener noreferrer" target="_blank">
      View on explorer
    </Link>
  );
};
