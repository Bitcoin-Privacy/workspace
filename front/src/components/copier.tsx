import { FC } from "react";
import { useClipboard, Spinner } from "@chakra-ui/react";
import { Text, Button } from "@chakra-ui/react";
import { FiCheck, FiCopy } from "react-icons/fi";

interface ICopier {
  content: string;
  maxW?: string;
}

export const Copier: FC<ICopier> = (props) => {
  const { content, maxW } = props;

  const { onCopy, hasCopied } = useClipboard(content);

  if (!content) {
    return <Spinner />;
  }

  return (
    <Button
      onClick={onCopy}
      variant="link"
      rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
      colorScheme="white"
    >
      <Text isTruncated maxW={maxW ?? "320px"}>
        {content}
      </Text>
    </Button>
  );
};
