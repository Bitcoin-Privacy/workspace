import { Box, HStack, ListItem, Text, UnorderedList } from "@chakra-ui/react";
import Moment from "moment";

import { RoomDto } from "@/dtos";
import { FC, useMemo } from "react";
import { CoinJoinRoomStatus } from "./coinjoin-room-status";

interface ICoinJoinRoomCard {
  key: any;
  data: RoomDto;
  deriv: string;
}

export const CoinJoinRoomCard: FC<ICoinJoinRoomCard> = (props) => {
  const { key, data, deriv } = props;

  const dues = useMemo(() => {
    return {
      endOfDue1: data.created_at + data.due1,
      endOfDue2: data.created_at + data.due1 + data.due2,
    };
  }, [data]);

  return (
    <HStack
      key={key}
      color="white"
      textAlign="start"
      w="100%"
      bg="#3a3a3a"
      p="8px 16px"
      borderRadius="8px"
      minW="300px"
      justifyContent="space-between"
    >
      <Box>
        <Text fontSize="16px" fontWeight="700" w="100%">
          Room {data.id.slice(undefined, 8)}
        </Text>
        <UnorderedList w="100%">
          <ListItem>Amount: {data.base_amount} sats</ListItem>
          <ListItem>Number of peers: {data.no_peer}</ListItem>
          <ListItem>Status: {data.status}</ListItem>
          <ListItem>
            {"Due 1: " + Moment(dues.endOfDue1).format("MMM DD YYYY, HH:mm")}
          </ListItem>
          <ListItem>
            {"Due 2: " + Moment(dues.endOfDue2).format("MMM DD YYYY, HH:mm")}
          </ListItem>
        </UnorderedList>
      </Box>
      <CoinJoinRoomStatus
        deriv={deriv}
        roomId={data.id}
        endOfDue1={data.created_at + data.due1}
        endOfDue2={data.created_at + data.due1 + data.due2}
      />
    </HStack>
  );
};
