from pymongo import MongoClient, errors
from bson.objectid import ObjectId
from tiktok.bot import Bot, AccountType


class FYPVideo():
    def __init__(self, author, description, likes, comments, bookmarks, shares):
        self.author = author
        self.description = description
        self.likes = likes
        self.comments = comments
        self.bookmarks = bookmarks
        self.shares = shares

    def insert_video(self, client: MongoClient, username):
        db = client["algoprober"]
        collection = db["fyp_videos"]

        try:
            collection.insert_one({"username": username, "author": self.author, "description": self.description, "likes": self.likes, "comments": self.comments, "bookmarks": self.bookmarks, "shares": self.shares})
        except:
            print("Failed to insert FYP video..")

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

    def update_status(self, username, status):
        collection = self.db["users"]

        user_record = {"username": username}
        update_operation = {"$set": {"status": status}}

        collection.update_one(user_record, update_operation)

    def add_data(self, url, description):
        collection = self.db["videos"]

        collection.create_index("url", unique=True)

        try:
            collection.insert_one({"url": url, "description": description})

        except errors.DuplicateKeyError:
            pass

