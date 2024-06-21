import { Spinner, Text, VStack } from "@chakra-ui/react";
import { FC } from "react";
import { UtxoCard } from "./utxo-card";
import { UtxoDto } from "@/dtos";

interface IUtxoList {
  isLoading: boolean;
  isError: boolean;
  data: UtxoDto[];
}
export const UtxoList: FC<IUtxoList> = (props) => {
  const { data, isLoading, isError } = props;
  if (isLoading)
    return (
      <VStack h="100%" w="100%">
        <Spinner />
      </VStack>
    );
  if (isError)
    return (
      <VStack h="100%" w="100%">
        <Text>Failed to fetch list UTXOs</Text>
      </VStack>
    );
  return (
    <VStack h="100%" w="100%">
      {data?.map((val, index) => <UtxoCard key={index} data={val} />)}
    </VStack>
  );
};
