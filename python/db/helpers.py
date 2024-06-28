from datetime import datetime
from pymongo import MongoClient
from tiktok.bot import Bot, AccountType

class Video():
    def __init__(self, author, description, likes, comments, bookmarks, shares, date, sound, url):
        self.author = author
        self.description = description
        self.likes = likes
        self.comments = comments
        self.bookmarks = bookmarks
        self.shares = shares
        self.date = date
        self.sound = sound
        self.url = url

def get_bot_type(bot_type: str):
    for account_type in AccountType:
        if account_type.name == bot_type.upper():
            return account_type.value


class AlgoproberDB():
    def __init__(self, logger):
        self.client = MongoClient("mongodb://127.0.0.1:27017")
        self.db = self.client["algoprober"]
        self.logger = logger

    def get_bots(self, bot_type: str) -> list:
        collection = self.db["users"]
        bot_list = list()
        bots = collection.find({"user_type": get_bot_type(bot_type)})

        for bot in bots:
            if bot:
                bot_obj = Bot(
                        id_=bot.get("_id"),
                        email=bot.get("email"),
                        username=bot.get("username"),
                        password=bot.get("password"),
                        type_=bot.get("user_type"),
                        status=bot.get("status"),
                        sessionid=bot.get("sessionid"),
                        db=self,
                    )

                bot_list.append(bot_obj)
            else:
                self.logger.error("failed to retrieve bots")

        return bot_list

    def add_type_to_bot(self, username, bot_type):
        collection = self.db["users"]

        user_record = {"username": username}
        update_operation = {"$set": {"user_type": bot_type}}

        collection.update_one(user_record, update_operation)

    def add_sessionid(self, username, sessionid):
        collection = self.db["users"]

        user_record = {"username": username}
        update_operation = {"$set": {"sessionid": sessionid}}

        collection.update_one(user_record, update_operation)

    def update_status(self, username, status):
        collection = self.db["users"]

        user_record = {"username": username}
        update_operation = {"$set": {"status": status}}

        collection.update_one(user_record, update_operation)

    def insert_hashtag_video(self, username, video: Video):
        collection = self.db["hashtag_videos"]

        try:
            collection.insert_one({"username": username, "author": video.author, "description": video.description, "likes": video.likes, "comments": video.comments, "bookmarks": video.bookmarks, "shares": video.shares, "date": video.date, "sound": video.sound, "url": video.url, "timestmap": datetime.now.isoformat()})
        except:
            print("[!] failed to insert hashtag video")

    def insert_fyp_video(self, username, video: Video):
        collection = self.db["fyp_videos"]

        try:
            collection.insert_one({"username": username, "author": video.author, "description": video.description, "likes": video.likes, "comments": video.comments, "bookmarks": video.bookmarks, "shares": video.shares, "date": video.date, "sound": video.sound, "url": video.url, "timestamp": datetime.now().isoformat()})
        except:
            print("[!] failed to insert fyp video")
