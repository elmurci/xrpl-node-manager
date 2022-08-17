import { EventType, Topic } from '@/enums';

export type WsMessage = {
    event: EventType,
    topic: Topic,
    data?: Record<string, unknown>,
};