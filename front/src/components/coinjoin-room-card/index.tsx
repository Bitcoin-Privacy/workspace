import {
  Box,
  Button,
  HStack,
  ListItem,
  Text,
  UnorderedList,
} from "@chakra-ui/react";
import Moment from "moment";

import { CoinJoinApi } from "@/apis";
import { RoomDto } from "@/dtos";
import { convertSatsToBtc } from "@/utils";
import moment from "moment";
import { useMemo } from "react";

interface CoinJoinRoomCardProps {
  key: any;
  data: RoomDto;
  deriv: string;
}

export function CoinJoinRoomCard(props: CoinJoinRoomCardProps) {
  const { key, data } = props;

  const phase = useMemo(() => {
    let diff = moment.now() - data.created_at;
    if (diff < data.due1) {
      return 1;
    } else {
      return 2;
    }
  }, [data]);

  return (
    <HStack
      key={key}
      color="white"
      textAlign="start"
      bg="#3a3a3a"
      p="8px 16px"
      borderRadius="8px"
      align="stretch"
      w="full"
      justifyContent="space-between"
    >
      <Box>
        <Text fontSize="16px" fontWeight="700" w="100%">
          Room {data.id.slice(undefined, 8)}
        </Text>
        <UnorderedList w="100%">
          <ListItem>Amount: {convertSatsToBtc(data.base_amount)} BTC</ListItem>
          <ListItem>Status: {data.status}</ListItem>
          <ListItem>Phase: {phase}</ListItem>
          <ListItem>
            Due 1: {Moment(data.created_at + data.due1).format("MMM DD, HH:mm")}
          </ListItem>
          <ListItem>
            Due 2:{" "}
            {Moment(data.created_at + data.due1 + data.due2).format(
              "MMM DD, HH:mm",
            )}
          </ListItem>
        </UnorderedList>
      </Box>
      <Button
        h="100%"
        onClick={() => {
          CoinJoinApi.signTxn(props.deriv, data.id);
        }}
      >
        Sign
      </Button>
    </HStack>
  );
}
