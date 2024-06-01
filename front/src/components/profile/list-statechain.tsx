import { StateCoinDto } from "@/dtos";
import { Spinner, Text, VStack } from "@chakra-ui/react";
import { FC } from "react";
import { StateChainCard } from "../statechain-card";

interface IListStateChain {
  isLoading: boolean;
  isError: boolean;
  deriv: string;
  data: StateCoinDto[];
}
export const ListStateChain: FC<IListStateChain> = (props) => {
  const { deriv, data, isLoading, isError } = props;
  if (isLoading)
    return (
      <VStack h="100%" w="100%">
        <Spinner />
      </VStack>
    );
  if (isError)
    return (
      <VStack h="100%" w="100%">
        <Text>Failed to fetch list of state chain</Text>
      </VStack>
    );
  return (
    <VStack h="100%" w="100%">
      {data?.map((val, index) => (
        <StateChainCard val={val} key={index} deriv={deriv} />
      ))}
    </VStack>
  );
};
